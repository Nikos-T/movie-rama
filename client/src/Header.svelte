<script>
    import Modal from './components/Modal.svelte';
    import SignUpForm from './SignUpForm.svelte';
    import LogInForm from './LogInForm.svelte';

    // These should be on a store instead of dispatched
    let user;

    const isLoggedIn = () => {
        return user != undefined;
    }

    let showLogInModal = false;
    let showSignUpModal = false;

    const toggleLogInModal = () => {
        showLogInModal = !showLogInModal;
    }

    const toggleSignUpModal = () => {
        showSignUpModal = !showSignUpModal;
    }

    const signedUp = (e) => {
        showSignUpModal = false;
        user = e.detail;
        console.log(user);
    }

    const loggedIn = (e) => {
        showLogInModal = false;
        user = e.detail;
        console.log(user);
    }

</script>

<header>
	<div class="corner-left">
        <h1>MovieRama</h1>
	</div>

	<div class="corner-right">

        {#if user == undefined}
            <button on:click={toggleLogInModal}>Log in</button>
            <button on:click={toggleSignUpModal}>Sign up</button>
        {:else}
            <button>Add movie</button>
        {/if}
	</div>
</header>

<Modal title="Log in" showModal={showLogInModal} on:click={toggleLogInModal}>
    <LogInForm on:loggedIn={loggedIn}/>
</Modal>
<Modal title="Sign up" showModal={showSignUpModal} on:click={toggleSignUpModal}>
    <SignUpForm on:signedUp={signedUp}/>
</Modal>

<style>
.corner-left {
    float: left;
}

.corner-right {
    float: right;
}
</style>
