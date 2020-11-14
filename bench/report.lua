done = function(summary, latency, requests)
   io.write("___\n")
   io.write(string.format("%d µs (+/- %d µs) | ", latency.mean, latency.stdev))
   io.write(string.format("%d (+/- %d) | ", requests.mean, requests.stdev))
   io.write(string.format("%d | ", summary.requests))
   io.write(string.format("%d | ", summary.bytes))
end