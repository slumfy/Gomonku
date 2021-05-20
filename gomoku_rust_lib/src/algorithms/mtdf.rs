// function MTDF(root, f, d) is
//     g := f
//     upperBound := +∞
//     lowerBound := −∞

//     while lowerBound < upperBound do
//         β := max(g, lowerBound + 1)
//         g := AlphaBetaWithMemory(root, β − 1, β, d)
//         if g < β then
//             upperBound := g 
//         else
//             lowerBound := g

//     return g
// f