<script>
// import bcrypt to hash password
import { createEventDispatcher } from 'svelte';
import UserStore from '../stores/UserStore.js';

let dispatch = createEventDispatcher();

let email;
let password;

const handleLogIn = async () => {
    const res = await fetch('/api/login', {
			method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
			body: JSON.stringify({
				email,
				password
			})
		});

    if (!res.ok) {
        const message = await res.text();
        alert(message);
        return;
    }

    let json_res = await res.json();

    UserStore.update(shouldBeEmpty => {
        // TODO return better json format from api
        return {
            user: json_res[0],
            token: json_res[1]
        }
    });

    dispatch('loggedIn')
};

</script>

<form on:submit|preventDefault={handleLogIn}>
    <input type="emai" name="email" placeholder="email" bind:value={email}/>
    <input type="password" name="password" placeholder="password" bind:value={password}/>
    <button>Log in</button>
</form>

<style>
</style>
