class OpenHanseShell extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: "open" });
    this.commandHandler = null;
    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: grid;
          grid-template-rows: auto 1fr auto;
          min-height: 100vh;
          border: 1px solid #111;
        }

        .frame {
          display: grid;
          grid-template-rows: auto 1fr auto;
          min-height: 100vh;
        }
      </style>
      <div class="frame">
        <oh-status></oh-status>
        <oh-log></oh-log>
        <oh-prompt></oh-prompt>
      </div>
    `;
  }

  connectedCallback() {
    this.prompt.addEventListener("command", (event) => {
      if (this.commandHandler) {
        this.commandHandler(event.detail.value);
      }
    });
  }

  get statusPanel() {
    return this.shadowRoot.querySelector("oh-status");
  }

  get logPanel() {
    return this.shadowRoot.querySelector("oh-log");
  }

  get prompt() {
    return this.shadowRoot.querySelector("oh-prompt");
  }

  setApiBase(apiBase) {
    this.statusPanel.setStaticLine(`API ${apiBase}`);
  }

  setStatus(status) {
    this.statusPanel.setStatus(status);
  }

  setInbox(inbox) {
    this.statusPanel.setInboxCount(inbox.length);
  }

  appendEvent(event) {
    this.logPanel.appendLine(`[${event.kind}] ${event.message}`);
  }

  appendLog(kind, message) {
    this.logPanel.appendLine(`[${kind}] ${message}`);
  }

  clearLog() {
    this.logPanel.clear();
  }

  onCommand(handler) {
    this.commandHandler = handler;
  }

  setFatalError(message) {
    this.appendLog("error", message);
  }
}

customElements.define("oh-shell", OpenHanseShell);
