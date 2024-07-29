from queue import Queue
from CdElemento import cdEncontra, cdLigar


class Graph:
    def __init__(self):
        self.verticesAmount = 0
        self.__file = None
        self.edgesAmount = 0
        self.adj_list = []
        self.matrix = []
        self.rotulos = []
        self.vertices = []
        self.edges = []

    def qtdVertices(self) -> int:
        return self.verticesAmount

    def qtdArestas(self) -> int:
        return self.edgesAmount

    def vertice(self, rotulo: str) -> int:
        return self.rotulos.index(rotulo)

    def grau(self, id: int) -> int:
        return len(self.adj_list[id-1])

    def rotulo(self, id: int):
        return self.rotulos[id-1]

    def vizinhos(self, id: int):
        return [x[0]+1 for x in self.adj_list[id-1]]

    def haAresta(self, id_u: int, id_v: int):
        if self.matrix[id_u-1][id_v-1] != float('inf'):
            return True
        return False

    def peso(self, id_u: int, id_v: int):
        return self.matrix[id_u-1][id_v-1]

    def lerComeco(self):
        # Define a quantidade de vértices conforme informado no arquivo lido
        self.verticesAmount = int(self.__file.readline().split()[1])

    def lerDefinicaoArestas(self):
        return self.__file.readline().strip("\n")

    def lerVertices(self):
        for i in range(self.verticesAmount):
            result = list(map(lambda x: x.replace("\"", ""), self.__file.readline().strip("\n").split(' ', 1)))
            value, identifier = result
            # Adiciona os rótulos dos vértices numa lista de rótulos, que são acessados
            # através da posição (id do vértice - 1) de self.rotulos
            self.rotulos.append(identifier)
            self.vertices.append(i+1)
    
    def lerArestas(self, tipo_arestas):
        while True:
            line = self.__file.readline()
            if line == '' or line == "\n":
                break
            result = line.strip("\n").split()
            fr, to, weight = result
            self.edgesAmount += 1
            # Coloca os pesos das arestas na matriz de adjacencia
            self.matrix[int(fr)-1][int(to)-1] = float(weight)
            if tipo_arestas == "*edges":
                self.matrix[int(to)-1][int(fr)-1] = float(weight)

            # Adiciona o vértice "to" e o peso da aresta que conecta "fr" e "to" na lista de ajacências de "fr"
            # e, se for grafo não dirigido, adiciona o vértice "fr" e peso da aresta que conecta
            # "fr" e "to" na lista de adjacências de "to"
            self.adj_list[int(fr)-1].append((int(to)-1, float(weight)))
            self.edges.append((int(fr)-1, int(to)-1, float(weight)))
            if tipo_arestas == "*edges":
                self.adj_list[int(to)-1].append((int(fr)-1, float(weight)))
                self.edges.append((int(to)-1, int(fr)-1, float(weight)))

    def ler(self, file: str):
        self.__file = open(file)
        self.lerComeco()

        self.criarMatriz()
        self.criarListaAdj()

        self.lerVertices()
        tipo_aresta = self.lerDefinicaoArestas()
        self.lerArestas(tipo_aresta)
        self.__file.close()

    def criarMatriz(self):
        # Cria matriz de adjacências |V|x|V| com todos elementos iguais a float('inf') inicialmente
        for i in range(self.verticesAmount):
            self.matrix.append([float('inf')] * (self.verticesAmount))

    def criarListaAdj(self):
        # Cria |V| listas de adjacências (de cada vértice) dentro de uma lista principal
        # Cada lista de adjacências é acessada através da posição (id do vértice - 1) de self.adj_list
        for i in range(self.verticesAmount):
            self.adj_list.append([])

    def buscaLargura(self, s):
        # Criando listas para as distancias, antecessores dos vértices e para indicar
        # se os vértices já foram visitados
        distancias = [float('inf') for i in range(self.verticesAmount)]
        antecessores = [None for i in range(self.verticesAmount)]
        visitados = [False for i in range(self.verticesAmount)]
        # Fila de visitas
        fila = Queue()
        fila.put(s)
        # Configurando vértice de origem
        visitados[s-1] = True
        distancias[s-1] = 0
        while not fila.empty():
            # Visita o vértice inserido na fila primeiro
            u = fila.get()
            # Percorre os vizinhos de "u", configurando os que ainda não foram visitados
            for v in self.vizinhos(u):
                if (not visitados[v-1]):
                    visitados[v-1] = True
                    distancias[v-1] = distancias[u-1] + 1
                    antecessores[v-1] = u-1
                    fila.put(v)
        return (distancias, antecessores)


    def hierholzer(self):
        C = []   # Lista que representa as arestas ainda não visitadas
        # Percorrendo a lista de adjacências para preencher C
        for u in range(1, self.qtdVertices()+1):
            for v in self.adj_list[u-1]:
                if (v[0]+1, u) not in C: # Não coloca a mesma aresta 2 vezes
                    C.append((u, v[0]+1))

        # Escolhe o primeiro vértice da primeira aresta do dicionário C
        v = C[0][0]

        # Busca o ciclo Euleriano
        r, Ciclo = self.buscarSubcicloEuleriano(v, C)

        # Se não existe ciclo Euleriano
        if not r:
            return False, None

        # Se existe aresta em C, o ciclo não passou por alguma aresta então não existe ciclo Euleriano
        if len(C) != 0:
            return False, None
                

        return True, Ciclo


    def buscarSubcicloEuleriano(self, v, C):
        Ciclo = [v]
        t = v
        while True:
            # Procura uma aresta em C (não visitada) conectada a "v"
            u = None
            for e in C:
                if e[0] == v:
                    u = e[1]
                    C.remove(e)   # Aresta visitada retirada de C
                    break
                if e[1] == v:
                    u = e[0]
                    C.remove(e)   # Aresta visitada retirada de C
                    break
            if u is None:
                return False, None
            
            v = u
            # Adiciona o vértice ao final do ciclo
            Ciclo.append(v)
            # Se o primeiro vértice do ciclo for igual ao vértice que foi adicionado
            # ao ciclo por último, sai do loop
            if t == v:
                break

        for x in Ciclo:
            # Verifica de existe alguma aresta não visitada conectada a um
            # vértice "x" pertencente ao ciclo
            for e in C:
                if e[0] == x or e[1] == x:
                    # Busca um subciclo Euleriano a partir de "x"
                    r, Ciclo_ = self.buscarSubcicloEuleriano(x, C)

                    # Se não existe subciclo Euleriano
                    if not r:
                        return False, None

                    # Inserindo Ciclo_ em Ciclo
                    pos = Ciclo.index(x)
                    del Ciclo[pos]
                    Ciclo[pos:pos] = Ciclo_

        return True, Ciclo


    def dijkstra(self, s: int):
        # Criando listas para as distancias, antecessores dos vértices e para indicar
        # se os vértices já foram visitados
        distancias = [float('inf') for i in range(self.verticesAmount)]
        antecessores = [None for i in range(self.verticesAmount)]
        visitados = [False for i in range(self.verticesAmount)]
        # Configurando vértice de origem
        distancias[s-1] = 0
        while not all(visitados):
            # Escolhe vértice com a menor distância dentre os que não foram visitados
            min = float('inf')
            u = None
            for (idx, value) in enumerate(distancias):
                if value < min and not visitados[idx]:
                    min = value
                    u = idx
            if (u is None):
                break
            
            visitados[u] = True

            # Para cada vértice vizinho de "u" não visitado, verifica e atualiza
            # sua distância caso ela for menor ou igual a distância de "u" + o peso da aresta (u,v)
            for v in self.vizinhos(u+1):
                if visitados[v-1]:
                    continue
                if distancias[v-1] > distancias[u] + self.matrix[u][v-1]:
                    distancias[v-1] = distancias[u] + self.matrix[u][v-1]
                    antecessores[v-1] = u
        return (distancias, antecessores)


    def floydWarshall(self):
        # Preenchendo a matriz D de distancias
        D = []
        for u in range(self.qtdVertices()):
            D.append([])
            for v in range(self.qtdVertices()):
                if u == v:
                    # Duv = 0 se u==v
                    D[u].append(0)
                else:
                    # Duv = peso da aresta que liga "u" a "v" se existir
                    # caso contrário será infinito
                    D[u].append(self.peso(u+1, v+1))

        # Atualização da matriz D através dos pesos das arestas para a determinação de caminhos mínimos
        for k in range(self.qtdVertices()):
            for u in range(self.qtdVertices()):
                for v in range(self.qtdVertices()):
                    D[u][v] = min(D[u][v], D[u][k] + D[v][k])
        return D
    
    def componentesFortementeConexas(self):
        A, F = self.dfs()

        adj_list_t = []
        for v in range(self.qtdVertices()):
            adj_list_t.append([])

        for v in range(self.qtdVertices()):
            for tupla in self.adj_list[v]:
                adj_list_t[tupla[0]].append((v, tupla[1]))
        
        grafoT = Graph()
        # O grafo transposto criado só irá atualizar os atributos que serão usados: verticesAmount, vertices e lista de adjacências
        # Outros atributos (por exemplo: matriz de adjacência) não serão atualizados para poupar tempo e espaço
        grafoT.verticesAmount = self.qtdVertices()
        grafoT.vertices = F
        grafoT.adj_list = adj_list_t

        At, Ft = grafoT.dfs()

        return At
        
    
    def dfs(self):
        C = [False]*self.qtdVertices()
        F = []
        A = [None]*self.qtdVertices()
        for u in self.vertices:
            if not C[u-1]:
                C, A, F = self.dfsVisit(u, C, A, F)
        return A, F
    
    def dfsVisit(self, v, C, A, F):
        C[v-1] = True
        for u in self.vizinhos(v):
            if not C[u-1]:
                A[u-1] = v
                C, A, F = self.dfsVisit(u, C, A, F)
        F = [v] + F
        return C, A, F

    def kruskall(self):
        A = []
        pais = []
        ranks = []
        for v in self.vertices:
            pais.append(v-1)
            ranks.append(v-1)
        E = sorted(self.edges, key=lambda x: x[2])
        for (u, v, w) in E:
            x = cdEncontra(u, pais)
            y = cdEncontra(v, pais)
            if x != y:
                A.append((u, v, w))
                cdLigar(x, y, pais, ranks)
        return A

    def ordenacaoTopologica(self):
        C = [False for v in self.vertices]
        ord = []
        for u in self.vertices:
            if not C[u-1]:
                self.dfsVisitOT(u-1, C, ord)
        return ord

    def dfsVisitOT(self, v: int, C: [int], ord: [int]):
        C[v] = True
        for u in self.vizinhos(v+1):
            if not C[u-1]:
                self.dfsVisitOT(u-1, C, ord)
        ord.insert(0, v)
