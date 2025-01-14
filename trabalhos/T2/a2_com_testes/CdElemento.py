def cdEncontra(vertice: int, pais: [int]):
    if pais[vertice] != vertice:
        pais[vertice] = cdEncontra(pais[vertice], pais)
    return pais[vertice]


def cdLigar(x: int, y: int, pais: [int], ranks: [int]):
    if ranks[x] > ranks[y]:
        pais[y] = x
    else:
        pais[x] = y
        if ranks[x] == ranks[y]:
            ranks[y] += 1
