<script>
// import bcrypt to hash password
import { createEventDispatcher } from 'svelte';
import UserStore from '../stores/UserStore.js';

let dispatch = createEventDispatcher();

let email;
let password;
let first_name;
let last_name;

const handleSignUp = async () => {
    const res = await fetch('/api/signup', {
			method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
			body: JSON.stringify({
				email,
				password,
                first_name,
                last_name
			})
		});

    if (!res.ok) {
        const message = await res.text();
        alert(message);
        return;
    }

    let json_res = await res.json();

    // TODO return better json format from api
    sessionStorage.setItem('jwtToken', json_res[1]);
    UserStore.set({ user: json_res[0] });

    dispatch('signedUp')
};

</script>

<form on:submit|preventDefault={handleSignUp}>
    <input type="emai" name="email" placeholder="email" bind:value={email}/>
    <input type="password" name="password" placeholder="password" bind:value={password}/>
    <input type="text" name="first_name" placeholder="first name" bind:value={first_name}/>
    <input type="text" name="last_name" placeholder="last name" bind:value={last_name}/>
    <button>Sign up</button>
</form>

<style>
</style>
