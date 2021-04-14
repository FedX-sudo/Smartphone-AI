using Flux
using CSVFiles
using DataFrames

data = DataFrame(load("data.csv"))

ps = Flux.params(data[3:end])
