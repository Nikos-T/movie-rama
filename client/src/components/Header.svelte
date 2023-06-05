<script>
    import Modal from '../shared/Modal.svelte';
    import SignUpForm from './SignUpForm.svelte';
    import LogInForm from './LogInForm.svelte';
    import AddMovieForm from './AddMovieForm.svelte';
    import UserStore from '../stores/UserStore.js';

    // These should be on a store instead of dispatched
    let showLogInModal = false;
    let showSignUpModal = false;
    let showAddMovieModal = false;

    const toggleLogInModal = () => {
        showLogInModal = !showLogInModal;
    }

    const toggleSignUpModal = () => {
        showSignUpModal = !showSignUpModal;
    }

    const toggleAddMovieModal = () => {
        showAddMovieModal = !showAddMovieModal;
    }

    const signedUp = (e) => {
        showSignUpModal = false;
    }

    const loggedIn = (e) => {
        showLogInModal = false;
    }

    const addedMovie = (e) => {
        showAddMovieModal = false;
    }
</script>

<Modal title="Log in" showModal={showLogInModal} on:click={toggleLogInModal}>
    <LogInForm on:loggedIn={loggedIn}/>
</Modal>
<Modal title="Sign up" showModal={showSignUpModal} on:click={toggleSignUpModal}>
    <SignUpForm on:signedUp={signedUp}/>
</Modal>
<Modal title="Add movie" showModal={showAddMovieModal} on:click={toggleAddMovieModal}>
    <AddMovieForm on:addedMovie={addedMovie}/>
</Modal>

<header>
	<div class="corner-left">
        <h1>MovieRama</h1>
	</div>

	<div class="corner-right">
        {#if $UserStore.user == null}
            <button on:click={toggleLogInModal}>Log in</button>
            <button on:click={toggleSignUpModal}>Sign up</button>
        {:else}
            <p>Welcome back {$UserStore.user.first_name} {$UserStore.user.last_name}</p>
            <button on:click={toggleAddMovieModal}>Add movie</button>
        {/if}
	</div>
</header>

<style>
header {
    background-color: #f2f2f2;
    padding: 10px;
    display: flex;
    justify-content: space-between;
    align-items: center;
}
/*
.corner-left {
    float: left;
}

.corner-right {
    float: right;
}
*/
</style>
