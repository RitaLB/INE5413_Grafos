from Graph import Graph
import sys


def printKruskall(arvore):
    peso_total = 0
    for idx, (from_v, to_v, weight) in enumerate(arvore):
        peso_total += weight
    print(peso_total)
    for idx, (from_v, to_v, weight) in enumerate(arvore):
        if idx == len(arvore) - 1:
            print(f"{from_v+1}-{to_v+1}")
        else:
            print(f"{from_v+1}-{to_v+1}, ", end="")


def main():
    grafo = Graph()
    grafo.ler(sys.argv[1])
    arvore = grafo.kruskall()
    printKruskall(arvore)


if __name__ == "__main__":
    main()
