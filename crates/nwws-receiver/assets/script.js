console.log(new Date() + 'Loaded')
var eventSource = new EventSource('sse');

let list = document.querySelector('.ceefox-msglist');

eventSource.addEventListener('meta', function(event) {
    let li = document.createElement('li');
    li.innerHTML = event.data
    list.appendChild(li)
})

// eventSource.addEventListener('alert', function(event) {
//     console.log(event)
//     let data = JSON.parse(event.data)
//     if (data.status == "Test") {
//         data.headline = "Test message"
//     }
//     let li = document.createElement('li');
//     li.textContent = data.headline
//     list.appendChild(li)

// })

eventSource.addEventListener('alert', function(event) {
    let data = JSON.parse(event.data)
    if (data.status == "Test") {
        data.headline = "Test message"
    }
    let li = document.createElement('li');
    let alem = createAlertElement(data)
    console.log(alem)
    li.innerHTML = alem
    list.appendChild(li)

})

eventSource.onopen = function() {
    console.log(new Date() + 'Opened strm');
}
eventSource.onerror = function(e) {
    console.log('error', e);
}

function createAlertElement(alert) {
    let info = extractInfos(alert)
    console.log(info)
    let res = `
        <article class="alert">
            <div class="alert-top">
                <span class="alert-id">${alert.id}</span> sent at <span class="alert-sent">${alert.sent}</span>
            </div>
            <div class="alert-info-container">
            ${info}
            </div>
            <div class="alert-api-link">
                <a href="https://api.weather.gov/alerts/${alert.id}">https://api.weather.gov/alerts/${alert.id}</a>
            </div>
        </article>
    `
    console.log(res)
    return res
}

function extractInfos(alert) {
    return alert.info.map(createInfoElement)
}

function createInfoElement(info, index) {
    let desc = nl2br(info.description)
    let inst = nl2br(info.instruction)
    return `
        <details class="alert-info">
            <summary><span class="alert-info-index">[${index}]</span> ${info.headline}</summary>
            <p class="alert-info-description">${desc}</p>
            <p class="alert-info-instructions">${inst}</p>
        </details>
    `
}

// From: https://gist.github.com/yidas/41cc9272d3dff50f3c9560fb05e7255e
function nl2br (str, replaceMode, isXhtml) {
    var breakTag = (isXhtml) ? '<br />' : '<br>';
    var replaceStr = (replaceMode) ? '$1'+ breakTag : '$1'+ breakTag +'$2';
    return (str + '').replace(/([^>\r\n]?)(\r\n|\n\r|\r|\n)/g, replaceStr);
}
function br2nl (str, replaceMode) {   
    var replaceStr = (replaceMode) ? "\n" : '';
    // Includes <br>, <BR>, <br />, </br>
    return str.replace(/<\s*\/?br\s*[\/]?>/gi, replaceStr);
}