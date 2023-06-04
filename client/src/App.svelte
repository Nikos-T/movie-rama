<script>
    import Header from './components/Header.svelte';
    import MovieStore from './stores/MovieStore.js';

    // update MovieStore from /api/movies
    fetch('/api/movies')
        .then(res => res.json())
        .then(data => {
            MovieStore.set(data);
            // MovieStore.update(movies => {
            //     movies = data;
            //     return movies;
            // });
        })
        .catch(err => console.log(err));
    
</script>

<main>
    <Header/>
    {#each $MovieStore as movie (movie.id)}
        <div>
            <h3>{movie.title}</h3>
            <p>{movie.description}</p>
        </div>
    {/each}
</main>

<style>
    /*
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
    */
</style>
