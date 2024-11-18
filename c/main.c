struct task { // define task as struct in struct namespace; access with `struct task`
    // task
    char * name;
    bool status;
    // blocking on tasks:
    struct task * deps;
};

typedef struct task task; // define task in global name space; access with `task`

int main(){
    // create task (add name, init status=0, connect to dependencies)
    // 

    return 0;
}