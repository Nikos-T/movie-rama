import { writable } from 'svelte/store';

const UserStore = writable({user: null, votes: []});

export default UserStore;


