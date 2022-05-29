class LspStdin {
  static create(stdin: HTMLTextAreaElement, sendButton: HTMLButtonElement): ReadableStream {
    const encoder = new TextEncoder();
    return new ReadableStream({
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
}

class LspStdout {
  static create(stdout: HTMLTextAreaElement): WritableStream {
    const decoder = new TextDecoder();
    return new WritableStream({
      async write(bytes) {
        const message = decoder.decode(bytes);
        const payload = message.replace(/^Content-Length:\s*\d+\s*/, "");
        stdout.value += payload;
        stdout.value += "\n";
      },
    });
  }
}

export { LspStdin, LspStdout };
