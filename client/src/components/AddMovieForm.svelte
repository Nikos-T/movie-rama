<script>
import { createEventDispatcher } from 'svelte';
import UserStore from '../stores/UserStore.js';
import MovieStore from '../stores/MovieStore.js';

let dispatch = createEventDispatcher();

let title;
let description;

const handleAddMovie = async () => {
    const res = await fetch('/api/add_movie', {
			method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer ' + sessionStorage.getItem('jwtToken')
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

    let movie_verbose = {
        description: json_res.description,
        movie_id: json_res.id,
        negative_votes: 0,
        positive_votes: 0,
        posted_at: json_res.posted_at,
        posted_by: json_res.posted_by,
        title: json_res.title,
        user_first_name: $UserStore.user.first_name,
        user_last_name: $UserStore.user.last_name,
    };

    MovieStore.update((movies) => {
        return [movie_verbose, ...movies];
    });

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
