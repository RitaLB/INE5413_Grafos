from Graph import Graph
import sys


def printOrdenacaoTopologica(grafo, lista):
    for idx, v in enumerate(lista):
        if idx == len(lista) - 1:
            print(f"{grafo.rotulo(v+1)}")
        else:
            print(f"{grafo.rotulo(v+1)} -> ", end="")


def main():
    grafo = Graph()
    grafo.ler(sys.argv[1])
    lista = grafo.ordenacaoTopologica()
    printOrdenacaoTopologica(grafo, lista)


if __name__ == "__main__":
    main()
