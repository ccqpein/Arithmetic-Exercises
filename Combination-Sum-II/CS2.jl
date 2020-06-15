function combinationSum2(cands, target)
    sort!(cands) # sort all cands
    listOfSets = [Set() for _ in 1:(target+1)]

    push!(listOfSets[1], []) # index 1 actually point to 0, because julia index start from 1

    for
        cand in cands,
        t in range(target, cand, step=-1),
        prev in listOfSets[t+1-cand];

        cache = copy(prev)
        push!(cache, cand)
        push!(listOfSets[t+1], cache)
    end
        
    last(listOfSets)
end
