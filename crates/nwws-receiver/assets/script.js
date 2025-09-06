console.log(new Date() + 'Loaded')
var eventSource = new EventSource('sse');

let list = document.querySelector('.ceefox-msglist');

eventSource.addEventListener('meta', function(event) {
    let li = document.createElement('li');
    li.innerHTML = event.data
    list.appendChild(li)
})

eventSource.addEventListener('alert', function(event) {
    console.log(event)
    let data = JSON.parse(event.data)
    if (data.status == "Test") {
        data.headline = "Test message"
    }
    let li = document.createElement('li');
    li.innerHTML = data.headline
    list.appendChild(li)
})
// eventSource.onmessage = function(event) {
//     let li = document.createElement('li');
//     let data = JSON.parse(event.data)
//     li.innerHTML = event.data.headline
//     list.appendChild(li)
// }
eventSource.onopen = function() {
    console.log(new Date() + 'Opened strm');
}
eventSource.onerror = function(e) {
    console.log('error', e);
}
