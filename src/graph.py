from pyvis.network import Network
import json

class Node():
    def __init__(self, id: int, name: str):
        self.id = id
        self.name = name
        self.done = False
        self.parent = None
        self.dependencies = []

    def __repr__(self):
        return f'[id: {self.id}, name: {self.name}, done: {self.done}]'

    def add_dependency(self, dependency: 'Node'):
        self.dependencies.append(dependency)
        dependency.set_parent(self)
    
    def set_parent(self, parent: 'Node'):
        self.parent = parent

    def set_done(self, status: bool):
        self.done = status

    def to_dict(self):
        return {
            'id': self.id,
            'name': self.name,
            'done': self.done,
            'dependencies': [d.id for d in self.dependencies]
        }

class Graph():
    def __init__(self):
        self.id = 1
        self.head = Node(0, 'root')
    
    def add_task(self, name: str, parent: Node) -> Node:
        node = Node(self.id, name)
        self.id += 1
        parent.add_dependency(node)
        return node
    
def save_graph(graph: Graph, filepath: str):
    nodes = {}
    stack = [graph.head]
    visited = set()

    while stack:
        node = stack.pop()
        if node.id in visited:
            continue
        visited.add(node.id)
        nodes[node.id] = node.to_dict()
        stack.extend(node.dependencies)
    
    with open(filepath, 'w') as f:
        json.dump(nodes, f)

def load_graph(filepath: str) -> Graph:
    with open(filepath, 'r') as f:
        data = json.load(f)
    
    id_to_node = {}
    for id, node in data.items():
        id_to_node[int(id)] = Node(int(node['id']), node['name'])
        id_to_node[int(id)].set_done(node['done'])
    
    for id, node in data.items():
        n = id_to_node[int(id)]
        for dep in node['dependencies']:
            n.add_dependency(id_to_node[dep])
    
    g = Graph()
    g.head = id_to_node[0]
    g.id = max(id_to_node) + 1
    return g

def visualize_graph_interactive(graph: Graph, output_file='graph.html'):
    net = Network(height='750px', width='100%', directed=True)
    stack = [graph.head]
    visited = set()

    while stack:
        node = stack.pop()
        if node.id in visited:
            continue
        visited.add(node.id)

        color = 'green' if node.done else 'red'
        label = f'{node.name} (id={node.id})'
        net.add_node(node.id, label=label, color=color)

        for dep in node.dependencies:
            if dep.id not in visited:
                color = 'green' if dep.done else 'red'
                label = f'{dep.name} (id={dep.id})'
                net.add_node(dep.id, label=label, color=color)
                stack.append(dep)    

            net.add_edge(node.id, dep.id)
            

    net.show_buttons(filter_=['nodes']) # edit node labels
    net.show(output_file, notebook=False)