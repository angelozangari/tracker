from graph import Graph, visualize_graph_interactive

def main():
    G = Graph()
    t1 = G.add_task('tracker', G.head)
    t2 = G.add_task('create prototype in python', t1)
    t3 = G.add_task('make graph editable with gui', t1)
    t4 = G.add_task('add deadlines', t1)
    t5 = G.add_task('add timeslots and sync with calendar', t1)
    t2.set_done(True)
    visualize_graph_interactive(G)


if __name__ == "__main__":
    main()
