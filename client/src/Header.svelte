<script>
    import Modal from './components/Modal.svelte';
    import SignUpForm from './SignUpForm.svelte';

    // This should be on a store
    let loggedIn = false;

    let showLogInModal = false;
    let showSignUpModal = false;

    const toggleLogInModal = () => {
        showLogInModal = !showLogInModal;
    }

    const toggleSignUpModal = () => {
        showSignUpModal = !showSignUpModal;
    }

</script>

<header>
	<div class="corner-left">
        <h1>MovieRama</h1>
	</div>

	<div class="corner-right">

        {#if !loggedIn}
            <button on:click={toggleLogInModal}>Log in</button>
            <button on:click={toggleSignUpModal}>Sign up</button>
        {:else}
            <button>Add movie</button>
        {/if}
	</div>
</header>

<Modal title="Log in" showModal={showLogInModal} on:click={toggleLogInModal}>
    <form>
        <input type="emai" id="email" placeholder="email"/>
        <input type="password" id="password" placeholder="password"/>
    </form>
</Modal>
<Modal title="Sign up" showModal={showSignUpModal} on:click={toggleSignUpModal}>
<SignUpForm/>
</Modal>

<style>
.corner-left {
    float: left;
}

.corner-right {
    float: right;
}
</style>
