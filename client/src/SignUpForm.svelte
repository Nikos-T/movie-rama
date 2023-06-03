<script>
// import bcrypt to hash password
import bcrypt from 'bcryptjs';

let email;
let password;
let first_name;
let last_name;

const handleSignUp = async () => {
    // test print hashed password:
    
    const salt = bcrypt.genSaltSync(10);
    const password_hash = bcrypt.hashSync(password, salt);
    console.log(password_hash);
    const res = await fetch('/api/create_user', {
			method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
			body: JSON.stringify({
				email,
				password_hash,
                first_name,
                last_name
			})
		});

    console.log(res);
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
