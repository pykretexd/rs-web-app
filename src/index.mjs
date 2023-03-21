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

let update = async () => {
    let response = await fetch('/api/cpus');
    if (response.status !== 200) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }

    let json = await response.json();

    render(html`<${App} cpus=${json}></${App}>`, document.body);
}

update();
setInterval(update, 200);
