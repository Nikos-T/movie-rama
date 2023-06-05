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

    // TODO return better json format from api
    sessionStorage.setItem('jwtToken', json_res[1]);
    UserStore.set({ user: json_res[0] });

    fetch('/api/get_my_votes', {
        headers: {
            'Authorization': 'Bearer ' + sessionStorage.getItem('jwtToken')
        }
    })
    .then(res => res.json())
    .then(data => {
        UserStore.update(store_data => {
            store_data.votes = data;
            return store_data;
        });
    })
    .catch(err => console.log(err));

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
