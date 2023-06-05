<script>
export let movie;

import UserStore from '../stores/UserStore.js';
import MovieStore from '../stores/MovieStore.js';
import { createEventDispatcher } from 'svelte';

let vote = $UserStore.votes.find(vote => vote.movie_id === movie.movie_id);
if (vote != null) {
    vote = vote.vote;
}
let dispatch = createEventDispatcher();

const userClicked = (user) => {
    dispatch('userClicked', user);
};

const handleThumbsUp = async () => {
    if (vote == "positive") {
        let res = await fetch("/api/delete_vote", {
            method: 'DELETE',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer ' + sessionStorage.getItem('jwtToken')
            },
            body: JSON.stringify({
                movie_id: movie.movie_id
            })
        });

        if (res.ok) {
            vote = null;

            // decrease positive_votes from movie
            MovieStore.update((movies) => {
                let movieToUpdate = movies.find(m => m.movie_id === movie.movie_id);
                movieToUpdate.positive_votes--;
                return movies;
            });
            // remove vote from $UserStore.votes
            UserStore.update((data) => {
                let voteIndex = data.votes.findIndex(v => v.movie_id === movie.movie_id);
                data.votes.splice(voteIndex, 1);
                return data;
            });
        }
    } else {
        let res = await fetch("/api/vote", {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer ' + sessionStorage.getItem('jwtToken')
            },
            body: JSON.stringify({
                movie_id: movie.movie_id,
                vote: "positive"
            })
        });

        if (res.ok) {
            if (vote == "negative") {
                // decrease negative votes first
                MovieStore.update((movies) => {
                    let movieToUpdate = movies.find(m => m.movie_id === movie.movie_id);
                    movieToUpdate.negative_votes--;
                    return movies;
                });
            }
            vote = "positive";

            // increase positive_votes for movie
            MovieStore.update((movies) => {
                let movieToUpdate = movies.find(m => m.movie_id === movie.movie_id);
                movieToUpdate.positive_votes++;
                return movies;
            });
            // add or edit vote from $UserStore.votes
            UserStore.update((data) => {
                let voteIndex = data.votes.findIndex(v => v.movie_id === movie.movie_id);
                if (voteIndex == -1) {
                    data.votes.push({
                        movie_id: movie.movie_id,
                        vote: "positive"
                    });
                } else {
                    data.votes[voteIndex].vote = "positive";
                }
                return data;
            });
        }

    }
};

const handleThumbsDown = async () => {
    if (vote == "negative") {
        let res = await fetch("/api/delete_vote", {
            method: 'DELETE',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer ' + sessionStorage.getItem('jwtToken')
            },
            body: JSON.stringify({
                movie_id: movie.movie_id
            })
        });

        if (res.ok) {
            vote = null;

            // decrease negative_votes from movie
            MovieStore.update((movies) => {
                let movieToUpdate = movies.find(m => m.movie_id === movie.movie_id);
                movieToUpdate.negative_votes--;
                return movies;
            });
            // remove vote from $UserStore.votes
            UserStore.update((data) => {
                let voteIndex = data.votes.findIndex(v => v.movie_id === movie.movie_id);
                data.votes.splice(voteIndex, 1);
                return data;
            });
        }
    } else {
        let res = await fetch("/api/vote", {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': 'Bearer ' + sessionStorage.getItem('jwtToken')
            },
            body: JSON.stringify({
                movie_id: movie.movie_id,
                vote: "negative"
            })
        });

        if (res.ok) {
            if (vote == "positive") {
                // Decrease positive votes first
                MovieStore.update((movies) => {
                    let movieToUpdate = movies.find(m => m.movie_id === movie.movie_id);
                    movieToUpdate.positive_votes--;
                    return movies;
                });
            }

            vote = "negative";

            // increase negative_votes for movie
            MovieStore.update((movies) => {
                let movieToUpdate = movies.find(m => m.movie_id === movie.movie_id);
                movieToUpdate.negative_votes++;
                return movies;
            });

            // add or edit vote from $UserStore.votes
            UserStore.update((data) => {
                let voteIndex = data.votes.findIndex(v => v.movie_id === movie.movie_id);
                if (voteIndex == -1) {
                    data.votes.push({
                        movie_id: movie.movie_id,
                        vote: "negative"
                    });
                } else {
                    data.votes[voteIndex].vote = "negative";
                }
                return data;
            });
        }
    }
};

</script>

<div>
    <h3>{movie.title}</h3>
    <p><i>Posted by 
        <span class="clickable" on:click={() => userClicked({id: movie.posted_by, first_name: movie.user_first_name, last_name: movie.user_last_name})}>
            {movie.user_first_name} {movie.user_last_name}
        </span>
    at {movie.posted_at}</i></p>
    <p>{movie.description}</p>
    {#if $UserStore.user == null || $UserStore.user.id == movie.posted_by}
        <p>{movie.positive_votes} likes | {movie.negative_votes} hates</p>
    {:else}
        <p>
            {movie.positive_votes} likes 
            <button on:click={handleThumbsUp} class:active={vote == "positive"}>👍</button>
            &nbsp;| {movie.negative_votes} hates
            <button on:click={handleThumbsDown} class:active={vote == "negative"}>👎</button>
        </p>
    {/if}
</div>

<style>
.clickable {
  cursor: pointer;
  color: blue;
  text-decoration: underline;
}

.clickable:hover {
  color: darkblue;
}
.active {
	background-color: lightblue;
}
</style>
