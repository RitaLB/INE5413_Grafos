from Graph import Graph
import sys

def printCFC(A):
    C = [False]*len(A)
    CFCs = dict()
    for v in range(len(A)):
        if not C[v]:
            CFCs, A, C, u = addVerticeCFC(CFCs, A, C, v)
    
    for componente in CFCs.values():
        print(",".join(map(str, componente)))

def addVerticeCFC(CFCs, A, C, v):
    if A[v] == None:
        if not C[v]:
            CFCs[v+1] = [v+1]
            C[v] = True
        return CFCs, A, C, v
    else:
        CFCs, A, C, pai = addVerticeCFC(CFCs, A, C, A[v]-1)
        if not C[v]:
            CFCs[pai+1] += [v+1]
            C[v] = True
        return CFCs, A, C, pai



def main():
    grafo = Graph()
    grafo.ler(sys.argv[1])
    A = grafo.componentesFortementeConexas()
    printCFC(A)


if __name__ == "__main__":
    main()