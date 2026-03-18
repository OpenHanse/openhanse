class OpenHanseStatus extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: "open" });
    this.staticLine = "";
    this.inboxCount = 0;
    this.status = null;
    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
          padding: 12px;
          white-space: pre-wrap;
        }
      </style>
      <div id="status"></div>
    `;
  }

  setStaticLine(value) {
    this.staticLine = value;
    this.render();
  }

  setInboxCount(value) {
    this.inboxCount = value;
    this.render();
  }

  setStatus(status) {
    this.status = status;
    this.render();
  }

  render() {
    const lines = [];
    if (this.status) {
      lines.push(`peer     ${this.status.peer_id}`);
      lines.push(`target   ${this.status.target_peer_id}`);
      lines.push(`server   ${this.status.server_base_url}`);
      lines.push(`direct   ${this.status.direct_base_url}${this.status.message_endpoint}`);
      lines.push(`heart    ${this.status.heartbeat_state}`);
      lines.push(`inbox    ${this.inboxCount}`);
      if (this.status.last_error) {
        lines.push(`error    ${this.status.last_error}`);
      }
    }
    if (this.staticLine) {
      lines.push(this.staticLine);
    }
    this.shadowRoot.querySelector("#status").textContent = lines.join("\n");
  }
}

customElements.define("oh-status", OpenHanseStatus);
