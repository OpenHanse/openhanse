class OpenHansePrompt extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: "open" });
    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
          padding: 12px;
        }

        form {
          display: grid;
          grid-template-columns: auto 1fr;
          gap: 8px;
          align-items: center;
        }

        input {
          width: 100%;
          border: 1px solid #111;
          padding: 8px;
          font: inherit;
          background: transparent;
          color: inherit;
        }
      </style>
      <form>
        <span>&gt;</span>
        <input type="text" autocomplete="off" spellcheck="false">
      </form>
    `;
  }

  connectedCallback() {
    this.shadowRoot.querySelector("form").addEventListener("submit", (event) => {
      event.preventDefault();
      const input = this.shadowRoot.querySelector("input");
      const value = input.value;
      input.value = "";
      this.dispatchEvent(new CustomEvent("command", {
        detail: { value },
        bubbles: true,
        composed: true
      }));
    });
  }
}

customElements.define("oh-prompt", OpenHansePrompt);
