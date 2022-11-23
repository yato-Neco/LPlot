
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

  var ctx = document.getElementById('mychart-scatter').getContext('2d');;
  let tmp_data = []

  

  await window.__TAURI__.event.listen('back-to-front', event => {
    console.log('back-to-front', event.payload);
    
    tmp_data = event.payload
    
    var myChart = new Chart(ctx, {
      type: 'bubble',
      data: {
        datasets: [{
          label: 'Lidar',
          data: [{x:0.0,y:0.0}],
          backgroundColor: '#f88',
        }],
      },
      options: {
        plugins: {
          streaming: {
            duration: 10000,
          },
        },
        scales: {
          y: { 
            min: -30, max: 30 ,
            type: 'realtime',
            realtime: {
              onRefresh: function(chart) {
    
                chart.data.datasets.forEach(function(dataset) {
                  event.payload.forEach(element => {
                    dataset.data.push(
                      { x: element[0], y: element[1] }
                    )
                  });
                });
              }
            }
          },
          x: { min: -30, max: 30 },
          
        },
      },
    });

    
  });


}

async function t() {
  console.log("a")
}

f();
t();
window.greet = greet;
