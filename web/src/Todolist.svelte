
<script>
    import Todo  from "./Todo.svelte";

    let lastId = 0;

    const createTodo = (text, done = false) => {
        return {
            id: lastId++,
            text,
            done
        };
    };

    let todoText = '';

    let todos = [
        createTodo("Learning svelete", false),
        createTodo("learning Spin", true)
    ];

    $: uncompletedCount = todos.filter(todo => !todo.done).length;
    $: status = `${uncompletedCount} of ${todos.length} remaining`;

    function archiveCompleted() {
        todos = todos.filter(todo => !todo.done);
    }

    function addTodo() {
        todos = todos.concat(createTodo(todoText));
        todoText = '';
    }

    function deleteTodo(id) {
        todos = todos.filter(todo => todo.id !== id);
    }

    function toggleTodo(todo) {
        todo.done = !todo.done;
    }

</script>

<div>
    <h2> To Do List</h2>
    <div>
        {status}
        <button on:click={archiveCompleted}>Archive Completed</button>
    </div>

    <form on:submit|preventDefault>
        <input
            size="30"
            placeholder="enter new todo item here"
            bind:value={todoText} />
        <button disabled={!todoText} on:click={addTodo}>Add</button>
    </form>

    <ul>
        {#each todos as todo}
            <Todo {todo} on:delete={() => deleteTodo(todo.id)} on:toggle={() => toggleTodo(todo)} />
        {/each}
    </ul>
</div>

<style>
    button {
        margin-left: 10px;
    }

    ul {
        list-style: none;   /* remove bullets */
        margin-left: 0;
        padding-left: 0;
    }
</style>