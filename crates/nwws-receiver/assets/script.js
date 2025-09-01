console.log(new Date() + 'Loaded')
var eventSource = new EventSource('sse');

let list = document.querySelector('.ceefox-msglist');

eventSource.onmessage = function(event) {
    let li = document.createElement('li');
    li.innerHTML = event.data
    list.appendChild(li)
}
eventSource.onopen = function() {
    console.log(new Date() + 'Opened strm');
}
eventSource.onerror = function(e) {
    console.log('error', e);
}
