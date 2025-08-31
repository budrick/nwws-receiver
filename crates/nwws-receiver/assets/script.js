console.log(new Date() + 'Loaded')
var eventSource = new EventSource('sse');

eventSource.onmessage = function(event) {
    console.log(JSON.parse(event.data));
}
eventSource.onopen = function() {
    console.log(new Date() + 'Opened strm');
}
eventSource.onerror = function(e) {
    console.log('error', e);
}
