
<script>
    import { getGuid, blurOnKey, sortOnName } from "./util";
    import Item from "./Item.svelte";

    export let category;
    export let categories;
    export let show;

    let editing = false;
    let itemName = '';
    let items = [];
    let message = '';

    $:items = Object.values(category.items);
    $:remaining = items.filter(item => !item.packed).length;
    $:total = items.length;
    $:status = `${remaining} of ${total} remaining`;
    $:itemsToShow = sortOnName(items.filter(i => shouldShow(show, i)));

    function shouldShow(show, item) {
        return (
            show === 'all' ||
            (show === 'packed' && item.packed) ||
            (show === 'unpacked' && !item.packed)
        );
    }

    function addItem() {
        const duplicate = Object.values(categories).some(cat => 
            Object.values(cat.items).some(item => item.name === itemName)
        );
        if (duplicate) {
            message = `The item "${itemName}" already exists yet.`;
            alert(message);
            return;
        }

        const {items} = category;
        const id = getGuid();
        items[id] = {id, name: itemName, packed: false};
        category.items = items;
        itemName = '';
    }

</script>

<section>
    <h3>
        {#if editing}
            <input
                bind:value={category.name}
                on:blur={() => (editing = false)}
                on:keypress={blurOnKey} />
        {:else}
            <span on:click={() => (editing = true)}>{category.name}</span>
        {/if}
        <span class="status">{status}</span>
        <button class="icon">&#x1F5D1;</button>
    </h3>
    <form on:submit|preventDefault={addItem}>
        <label>
            New Item
            <input required bind:value={itemName} />
        </label>
        <button>Add Item</button>
    </form>

    <ul>
        {#each itemsToShow as item (item.id)}
            <Item bind:item />
        {:else}
            <div>This category does not contain any items yet.</div>
        {/each}
    </ul>
</section>

<style>
    button,
    input {
      border: solid lightgray 1px;
    }
  
    button.icon {
      border: none;
    }
  
    h3 {
      display: flex;
      justify-content: space-between;
      align-items: center;
  
      margin: 0;
    }
  
    section {
      --padding: 0.5rem;
  
      background-color: white;
      border: solid transparent 3px;
      border-radius: var(--padding);
      color: black;
      display: inline-block;
      margin: var(--padding);
      padding: calc(var(--padding) * 2);
      padding-top: 0;
      vertical-align: top;
    }
  
    .status {
      font-size: 1.2rem;
      font-weight: normal;
      margin: 0 1rem;
    }
  
    ul {
      list-style: none;
      margin: 0;
      padding-left: 0;
    }
  </style>


