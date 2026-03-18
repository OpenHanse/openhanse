class OpenHanseLog extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: "open" });
    this.lines = [];
    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
          overflow: auto;
          border-top: 1px solid #111;
          border-bottom: 1px solid #111;
          padding: 12px;
          white-space: pre-wrap;
        }
      </style>
      <div id="lines"></div>
    `;
  }

  appendLine(line) {
    this.lines.push(line);
    this.render();
  }

  clear() {
    this.lines = [];
    this.render();
  }

  render() {
    this.shadowRoot.querySelector("#lines").textContent = this.lines.join("\n");
    this.scrollTop = this.scrollHeight;
  }
}

customElements.define("oh-log", OpenHanseLog);
