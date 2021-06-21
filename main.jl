print("welcom to my AI. \n \nloading the modules... \n\n")
print("loading CSV...           "); using CSV; print("done! \n")
print("loading dataframes...    "); using DataFrames; print("done! \n")
print("loading Flux..."); using Flux; print("....")
using Flux.Data: DataLoader; print(".")
using Flux: onehotbatch, onecold, @epochs; print("...")
using Flux.Losses: logitcrossentropy; print(". done!\n")
print("laoding base...          "); using Base: @kwdef; print("done! \n")
print("loading statistics...    "); using Statistics; print("done! \n")
print("loading random...        "); using Random; print("done! \n")
print("loading CUDA...          "); using CUDA; print("done! \n")
# yah ^^^thats^^^ a mess, but it looks realy pretty when compiling, so...
print("Modules loaded! \n")
CUDA.allowscalar(true) #should I use scallars, no, is this how this is meant to be used, no. do I care, no.

print("loading the file... ")
tf = CSV.read("data.csv", DataFrame) # this is a 29440X74 dataframe.
df = CuArray{Float64}(undef, (29440, 74)) # this is a 29440X74 dataframe full of 0's. Feel free to print it if you ever wondered what 2178560 0's looked like

print("creating the CUDA Array ")

for x in (1:29439) # yes, I know this is a very bad way of doing this, I don't care------------------------------------------|
    print("here") #                                                                                                          |
    numberofRowsskiped = 0 #                                                                                                 |
    y = 0; print("\n\n", x, y, "\n\n") #                                                                                     |
    print("starting row: ", x) #                                                                                             |
    for y in (1:75) # ----------------------------------------------------------------------------------------------------|L |
      if (y == 75) # ---------------------------|                                                                         |O |
        print("\n\ndone with row. ", x, "\n")#  |IF statment                                                              |O |
        break #                                 |                                                                         |P |
      end # ------------------------------------|                                                                         |  |
      print("\nat point: ", x," x ", y, " vallue:  ", tf[abs(x),abs(y)], " number of rows skiped: ", numberofRowsskiped) #|Y | Loop x
	    if typeof(tf[abs(x),abs(y)]) == String #|                                                                           |  |
                numberofRowsskiped+=1 #  |IF statment                                                                     |  |
                y += 1 #                 |                                                                                |  |
                print("\nskipping row") #|                                                                                |  |
      end #------------------------------|                                                                                |  |
        end #                                                                                                             |  |
        df[abs(x),y] += tf[abs(x),y] #                                                                                    |  |
    end # ----------------------------------------------------------------------------------------------------------------|  |
end # -----------------------------------------------------------------------------------------------------------------------|

print("done!")

print("\n \none hot encoding the data... ")
# TODO uncomment this before finnishing. I just did not want to go throught that onehot thing every time I run the program because it is increadibly ineficant and took too dang long.
# TODO this probobly does not work anymore
# TODO stop writing so many TODO(s)
#for i in 1:74

#    print("itteration: ", i, "... ")

#   Flux.onehotbatch(df[!, i], unique(df[!, i]))

#  print("Done! \n")

#end

print("one hot encoding completed\n")



sample = randsubseq(1:size(df,1), 0.75)
train = df[sample, :]
notsample = [i for i in 1:size(df,1) if isempty(searchsorted(sample, i))]
test = df[notsample, :]


function build_model(; imgsize=(28,28,1), nclasses=10)
    return Chain(
            Dense(prod(imgsize), 32, relu),
            Dense(32, nclasses))
end

@kwdef mutable struct Args
    η::Float64 = 3e-4       # learning rate, is anyoneone else questioning the fact that they decided to use a character that is not on the standard keyboard to represent this?
    batchsize::Int = 256    # batch size, whatever that means.
    epochs::Int = 10        # Yah, that seems accurate, because the only way to measure the mind numbingly long time it takes for an AI to compile can only be measured in gological time.
    use_cuda::Bool = true   # use gpu (if cuda available) in which case I feel your pain
end


args = Args(; ) # collect options in a struct for convenience

if CUDA.functional() && args.use_cuda
    @info "Training on CUDA GPU. I feel your pain (assuming you are on Linux of course)."
    CUDA.allowscalar(false)
    device = gpu
else
    @info "Training on CPU. This will take some time (unless its some stupid Threadripper or something)."
    device = cpu
end # this is the ending of the else statment.
# remember, always save your file before revcompiling.
print("\nCreating test and train dataloaders") # Um yah that.
# train_loader, test_loader = getdata(args, device) # this might be important? <_<

print("\nConstructing model.") # whatever that does
model = build_model() |> device # as this line of code is executing, an AI which will evently grow to destroy the world is being created...
ps = Flux.params(model); print("\ndefining model's trainable parameters") # yah no idea what this is doing, but its what the internet siad I needed and the internet is never wrong.

print("\ndefining the Optimizer") # yes this thing optimizes the other thing
opt = ADAM(args.η) # again, not a character on the standard keyboard. Who though that was a good idea.

print("\ntrainign the model ...") # completly abratrary comment: commencing modle trainng.

# TODO: Fix all of this VVVVVVVVVVVVVVV
for epoch in 1:args.epochs
    print("\n training on Epoch", epoch)
    # TODO: make this two for loops in one another so that it can train on x and y or something like that.
    for i in (1:29440*74) # AKA a realy long time.
        print("\ntransfering data to device") # so I am now loading the GPU';s vRAM with a 50M file, I sure hope you don't have a GPU wimpier than mine.
        train_ondev = device(train) # yah this is probobly realy bad, so I shoud make a TODO to change this, but I have no idea what I am doign so I will just leave it.
        print("\ncomputing the gradiant") # that just sounds cool
        gs = gradient(() -> (model(test)), ps) # that
        print("\nupdating modle paramaters") # Yes, I am aware that at this point my program is more print statments than AI
        Flux.Optimise.update!(opt, ps, gs) # read the print statment in the above line.
    end

    # Report on train and test
    train_loss, train_acc = loss_and_accuracy(train_loader, model, device)
    test_loss, test_acc = loss_and_accuracy(test_loader, model, device)
    println("Epoch=$epoch")
    println("  train_loss = $train_loss, train_accuracy = $train_acc")
    println("  test_loss = $test_loss, test_accuracy = $test_acc")
end
