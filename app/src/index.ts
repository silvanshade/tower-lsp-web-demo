class LspStdin {
  #stream: ReadableStream;

  constructor(stdin: HTMLTextAreaElement, sendButton: HTMLButtonElement) {
    const encoder = new TextEncoder();
    this.#stream = new ReadableStream({
      type: "bytes" as any,
      async start(controller) {
        while (true) {
          await new Promise<void>((resolve) => {
            sendButton.addEventListener(
              "click",
              () => {
                const payload = stdin.value;
                const message = `Content-Length: ${payload.length}\r\n\r\n${payload}`;
                const bytes = encoder.encode(message);
                controller.enqueue(bytes);
                stdin.value = "";
                resolve();
              },
              { once: true }
            );
          });
        }
      },
    });
  }

  getReader(): ReadableStreamDefaultReader {
    return this.#stream.getReader();
  }
}

class LspStdout {
  #stream: WritableStream;

  constructor(stdout: HTMLTextAreaElement) {
    const decoder = new TextDecoder();
    this.#stream = new WritableStream({
      async write(bytes) {
        const message = decoder.decode(bytes);
        const payload = message.replace(/^Content-Length:\s*\d+\s*/, "");
        stdout.value += payload;
        stdout.value += "\n";
      },
    });
  }

  getWriter(): WritableStreamDefaultWriter {
    return this.#stream.getWriter();
  }
}

export { LspStdin, LspStdout };
