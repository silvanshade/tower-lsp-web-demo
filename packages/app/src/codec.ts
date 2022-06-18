import * as vsrpc from "vscode-jsonrpc";

import Queue from "./queue";
import Tracer from "./tracer";

const encoder = new TextEncoder();
const decoder = new TextDecoder();

export class Bytes {
  static encode(input: string): Uint8Array {
    return encoder.encode(input);
  }

  static decode(input: Uint8Array): string {
    return decoder.decode(input);
  }
}

export class Headers {
  static add(message: string): string {
    return `Content-Length: ${message.length}\r\n\r\n${message}`;
  }

  static remove(delimited: string): string {
    return delimited.replace(/^Content-Length:\s*\d+\s*/, "");
  }
}

export class PromiseMap<K, V extends { toString(): string }> {
  #map: Map<K, PromiseMap.type<V>> = new Map();

  get(key: K & { toString(): string }): null | Promise<V> {
    let initialized: PromiseMap.type<V>;
    // if the entry doesn't exist, set it
    if (!this.#map.has(key)) {
      initialized = this.#set(key);
    } else {
      // otherwise return the entry
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      initialized = this.#map.get(key)!;
    }
    // if the entry is a pending promise, return it
    if (initialized.status === "pending") {
      return initialized.promise;
    } else {
      // otherwise return null
      return null;
    }
  }

  #set(key: K, value?: V): PromiseMap.type<V> {
    if (this.#map.has(key)) {
      // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
      return this.#map.get(key)!;
    }
    // placeholder resolver for entry
    let resolve = (item: V) => {
      void item;
    };
    // promise for entry (which assigns the resolver
    const promise = new Promise<V>((resolver) => {
      resolve = resolver;
    });
    // the initialized entry
    const initialized: PromiseMap.type<V> = { status: "pending", resolve, promise };
    if (null != value) {
      initialized.resolve(value);
    }
    // set the entry
    this.#map.set(key, initialized);
    return initialized;
  }

  set(key: K & { toString(): string }, value: V): this {
    const initialized = this.#set(key, value);
    // if the promise is pending ...
    if (initialized.status === "pending") {
      // ... set the entry status to resolved to free the promise
      this.#map.set(key, { status: "resolved" });
      // ... and resolve the promise with the given value
      initialized.resolve(value);
    }
    return this;
  }

  get size(): number {
    return this.#map.size;
  }
}

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace PromiseMap {
  export type type<V> = { status: "pending"; resolve: (item: V) => void; promise: Promise<V> } | { status: "resolved" };
}

// FIXME: tracing effiency
export class IntoServer extends Queue<Uint8Array> implements AsyncGenerator<Uint8Array, never, void> {
  enqueue(item: Uint8Array): void {
    Tracer.client(Headers.remove(decoder.decode(item)));
    super.enqueue(item);
  }
}

export interface FromServer extends WritableStream<Uint8Array> {
  readonly responses: { get(key: number | string): null | Promise<vsrpc.ResponseMessage> };
  readonly notifications: AsyncGenerator<vsrpc.NotificationMessage, never, void>;
  readonly requests: AsyncGenerator<vsrpc.RequestMessage, never, void>;
}

// eslint-disable-next-line @typescript-eslint/no-namespace
export namespace FromServer {
  export function create(): FromServer {
    return new StreamDemuxer();
  }
}

export class StreamDemuxer extends Queue<Uint8Array> {
  readonly responses: PromiseMap<number | string, vsrpc.ResponseMessage> = new PromiseMap();
  readonly notifications: Queue<vsrpc.NotificationMessage> = new Queue<vsrpc.NotificationMessage>();
  readonly requests: Queue<vsrpc.RequestMessage> = new Queue<vsrpc.RequestMessage>();

  readonly #start: Promise<void>;

  constructor() {
    super();
    this.#start = this.start();
  }

  // FIXME: we needs to actually do framed reads here since `bytes` may not be a complete message
  private async start(): Promise<void> {
    for await (const bytes of this) {
      const delimited = Bytes.decode(bytes);
      const message = JSON.parse(Headers.remove(delimited)) as vsrpc.Message;
      Tracer.server(message);
      if (vsrpc.Message.isResponse(message) && null != message.id) {
        this.responses.set(message.id, message);
        continue;
      }
      if (vsrpc.Message.isNotification(message)) {
        this.notifications.enqueue(message);
        continue;
      }
      if (vsrpc.Message.isRequest(message)) {
        this.requests.enqueue(message);
        continue;
      }
    }
  }
}
