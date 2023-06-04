<script>
import { createEventDispatcher } from 'svelte';
import UserStore from '../stores/UserStore.js';

let dispatch = createEventDispatcher();

let title;
let description;

const handleAddMovie = async () => {
    const res = await fetch('/api/add_movie', {
			method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer ' + $UserStore.token
            },
			body: JSON.stringify({
				title,
				description
			})
		});

    if (!res.ok) {
        const message = await res.text();
        alert(message);
        return;
    }

    let json_res = await res.json();

    dispatch('addedMovie')
};

</script>

<form on:submit|preventDefault={handleAddMovie}>
    <input type="text" name="title" placeholder="title" bind:value={title}/>
    <textarea name="description" placeholder="description" bind:value={description}/>
    <button>Add movie</button>
</form>

<style>
</style>
