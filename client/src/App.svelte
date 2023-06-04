<script>
    import Header from './components/Header.svelte';
    import MovieStore from './stores/MovieStore.js';
    import _ from 'lodash';

    // update MovieStore from /api/movies
    fetch('/api/movies_verbose')
        .then(res => res.json())
        .then(data => {
            MovieStore.set(data);
        })
        .catch(err => console.log(err));
    
    let param = "posted_at";
    let order = "desc";

    let date_text = "date ↑";
    let likes_text = "likes";
    let hates_text = "hates";

    let filter_user = null;

    function updateBtnText() {
        date_text = "date";
        likes_text = "likes";
        hates_text = "hates";
        if (param == "posted_at") {
            if (order == "asc") {
                date_text = "date ↓";
            } else {
                date_text = "date ↑";
            }
        } else if (param == "positive_votes") {
            if (order == "asc") {
                likes_text = "likes ↓";
            } else {
                likes_text = "likes ↑";
            }
        } else if (param == "negative_votes") {
            if (order == "asc") {
                hates_text = "hates ↓";
            } else {
                hates_text = "hates ↑";
            }
        }
    }

    const sortByLikes = () => {
        if (param == "positive_votes" && order == "asc") {
            param = "positive_votes";
            order = "desc";
        } else {
            param = "positive_votes";
            order = "asc";
        }
        updateBtnText();
    }

    const sortByHates = () => {
        if (param == "negative_votes" && order == "asc") {
            param = "negative_votes";
            order = "desc";
        } else {
            param = "negative_votes";
            order = "asc";
        }
        updateBtnText();
    }

    const sortByDate = () => {
        if (param == "posted_at" && order == "asc") {
            param = "posted_at";
            order = "desc";
        } else {
            param = "posted_at";
            order = "asc";
        }
        updateBtnText();
    }
</script>

<main>
    <Header/>
    <div>
        <button on:click={sortByLikes}>{likes_text}</button>
        <button on:click={sortByHates}>{hates_text}</button>
        <button on:click={sortByDate}>{date_text}</button>
    </div>
    <!-- TODO: Filter before ordering -->
    {#if filter_user != null}
    <h4>Showing movies added by {filter_user.first_name} {filter_user.last_name} <span class="clickable" style="color: red;" on:click={() => {filter_user = null;}}><b><u>✖</u></b></span></h4>
    {/if}
    {#each _.orderBy($MovieStore, [param], [order]) as movie (movie.movie_id)}
    <!-- {#each $MovieStore as movie (movie.id)} -->
        {#if filter_user == null || filter_user.id == movie.posted_by}
        <div>
            <h3>{movie.title}</h3>
            <p><i>Posted by 
                <span class="clickable" on:click={() => {filter_user = {id: movie.posted_by, first_name: movie.user_first_name, last_name: movie.user_last_name};}}>
                    {movie.user_first_name} {movie.user_last_name}
                </span>
            at {movie.posted_at}</i></p>
            <p>{movie.description}</p>
            <p>{movie.positive_votes} likes | {movie.negative_votes} hates</p>
        </div>
        {/if}
    {/each}
</main>

<style>
.clickable {
  cursor: pointer;
  color: blue;
  text-decoration: underline;
}

.clickable:hover {
  color: darkblue;
}
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
