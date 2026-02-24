<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  type ElementType = 'input' | 'textarea' | 'select';
  type Variant = 'default' | 'ghost' | 'error';
  type Size = 'sm' | 'md' | 'lg';

  export let as: ElementType = 'input';
  export let variant: Variant = 'default';
  export let size: Size = 'md';
  export let value: string | number | undefined = undefined;
  export let type = 'text';
  export let rows = 3;
  export let className = '';

  const dispatch = createEventDispatcher<{
    input: Event;
    change: Event;
    blur: FocusEvent;
    focus: FocusEvent;
  }>();

  const base =
    'input-base text-sm';
  const variants: Record<Variant, string> = {
    default: '',
    ghost: 'input-ghost',
    error: 'input-error'
  };
  const sizes: Record<Size, string> = {
    sm: 'px-2.5 py-1.5 text-xs',
    md: 'px-3 py-2 text-sm',
    lg: 'px-4 py-2.5 text-base'
  };

  $: classes = `${base} ${variants[variant]} ${sizes[size]} ${className}`.trim();

  function onInput(event: Event) {
    dispatch('input', event);
  }

  function onChange(event: Event) {
    dispatch('change', event);
  }

  function onBlur(event: FocusEvent) {
    dispatch('blur', event);
  }

  function onFocus(event: FocusEvent) {
    dispatch('focus', event);
  }
</script>

{#if as === 'textarea'}
  <textarea
    class={classes}
    rows={rows}
    bind:value
    on:input={onInput}
    on:change={onChange}
    on:blur={onBlur}
    on:focus={onFocus}
    {...$$restProps}
  >
    <slot />
  </textarea>
{:else if as === 'select'}
  <select
    class={classes}
    bind:value
    on:input={onInput}
    on:change={onChange}
    on:blur={onBlur}
    on:focus={onFocus}
    {...$$restProps}
  >
    <slot />
  </select>
{:else}
  <input
    class={classes}
    type={type}
    bind:value
    on:input={onInput}
    on:change={onChange}
    on:blur={onBlur}
    on:focus={onFocus}
    {...$$restProps}
  />
{/if}
