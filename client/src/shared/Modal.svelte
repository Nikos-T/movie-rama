<script>

export let showModal = false;
export let title = "";

const hideModal = () => {
    showModal = false;
}
</script>

{#if showModal}
<div class="backdrop" on:click|self>
    <div class="modal">
        <h3>{title}</h3>
        <slot></slot>
    </div>
</div>
{/if}

<style>
.backdrop{
    width: 100%;
    height: 100%;
    position: fixed;
    background: rgba(0,0,0,0.8);
}

.modal{
    padding: 10px;
    border-radius: 10px;
    max-width: 400px;
    margin: 10% auto;;
    text-align: center;
    background: white;
}
</style>
