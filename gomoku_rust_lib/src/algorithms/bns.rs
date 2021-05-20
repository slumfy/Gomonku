// function nextGuess(α, β, subtreeCount) is
//     return α + (β − α) × (subtreeCount − 1) / subtreeCount


// function bns(node, α, β) is
//     subtreeCount := number of children of node

//     do
//         test := nextGuess(α, β, subtreeCount)
//         betterCount := 0
//         for each child of node do
//             bestVal := −alphabeta(child, −test, −(test − 1))
//             if bestVal ≥ test then
//                 betterCount := betterCount + 1
//                 bestNode := child
//         (update number of sub-trees that exceeds separation test value)
//         (update alpha-beta range)
//     while not (β − α < 2 or betterCount = 1)

//     return bestNode