import { h, render } from 'https://esm.sh/preact';
import htm from 'https://esm.sh/htm';

const html = htm.bind(h);

function App(props) {
    return html`
        <div>
            ${props.cpus.map((cpu) => {
                return html`
                    <div class="bar">
                        <label>${cpu.toFixed(2)}%</label>
                        <div class="bar-inner" style="width: ${cpu}%"></div>
                    </div>
                `;
            })}
        </div>
    `;
}

let url = new URL("/realtime/cpus", window.location.href);
url.protocol = url.protocol.replace("http", "ws");

let ws = new WebSocket(url.href);
ws.onmessage = (ev) => {
    let json = JSON.parse(ev.data);
    render(html`<${App} cpus=${json}></${App}>`, document.body);
};
