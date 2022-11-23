const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
});

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function f() {

  var ctx = document.getElementById("mychart-scatter").getContext("2d");

  var myChart = new Chart(ctx, {
    type: "bubble",
    data: {
      datasets: [
        {
          label: "Lidar",
          data: [],
          backgroundColor: "#f88",
        },
      ],
    },
    options: {
      animation: false,
      plugins: {
        streaming: {
          duration: 0,
        },
      },
      scales: {
        y: { min: -800, max: 800 },
        x: { min: -800, max: 800 },
      },
    },
  });

  await window.__TAURI__.event.listen("back-to-front", (event) => {
    //console.log("back-to-front", event.payload);

    

    let tmp_data = [];

    event.payload.forEach((element) => {
      tmp_data.push({ x: element[0], y: element[1] });
    });

    myChart.data.datasets[0].data = tmp_data

    myChart.update({
      lazy: true,
      duration: 0,
    });
  });
}

async function t() {
  console.log("a");
}

f();
t();
window.greet = greet;
