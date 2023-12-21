# Templates CLI

Have You ever wanted to create new NextJS route which let You use query parameters using `useSearchParams` hook?  
Or insert a server side page exporting load and action functions for SvelteKit with new `satisfies` operator?  
Or make a Java Spring Controller files with all the @Annotations and routes set for GET, POST, PUT ready?

And You always needed to check the documentation, because You forget how excatly the pages looks like? Which hooks to use, what types to import, how to properly name the functions?

If yes, then <B>Templates CLI</b> is for You :).

It's a simple tool that let's You very easly create, manage and use a set of templates pages for Your projects.

Full written in <b>Rust</b> (needed to point it out, cos this is why You learn Rust, right?).

## Install

```
cargo install templates
```

Please make sure that `~/.cargo/bin` is in your PATH.  
In the future more distro related options will be added :)

## Best way to learn is by example!

```
tp copy p ps -p reports -- svelte
```

This will copy two files that You have previously created in the

- ~/templates/svelte/[p]+page.svelte
- ~/templates/svelte/[ps]+page.server.ts

into:

- ./reports/+page.svelte
- ./reports/+page.server.ts

each containing their own template You prepared beforehand (containing types, function, whatever You want).

```
tp show ls
```

This will show the content of the file `~/templates/[ls]your_file_name` in the terminal, ready to be copied.

```
tp var -- next
```

This will list all the env variables that are listed in `~/templates/next/var` file.

## Usage

### Set

First thing You need to do is set up a `templates` folder. Default value is `~/templates`.

```
tp set [path_to_your_templates_folder]
```

Inside start creating Your template files with a format `[cli_shortcut]template_name`.
You can group them by subfolders (e.g. `svelte`, `next`).

Few examples:

- `[r]readme.md`
- `[c]config.json`
- `[p]+page.svelte`
- `svelte/[ls]+layout.server.ts`
- `svelte/[e]+error.svelte`
- `next/[p]page.ts`
- `next/[lt]layout.ts`
- `next/[lj]layout.js`

### Copy

```
tp copy [pages] -p <path> -- <project>
```

- `pages` - list of shortcut for template files (e.g. `p`, `ls`).
- `path` - path for where the files will be created (optional).
- `project` - the name of the subfolder inside Your templates folder (optional).

Using example settings from config section:

```
tp copy p lj -- next
```

This will create a `page.ts` and `layout.js` in `./` dir and copy content of `next/[p]page.ts` and `next/[lj]layout.js` respectively inside them.

### Show

```
tp show [page] -- <project>
```

- `page` - list of shortcut for template files (e.g. `p`, `ls`).
- `project` - the name of the subfolder inside Your templates folder (optional).

```
tp show p
```

This will show the content of the `[p]file_name` template file in your terminal.

### Var

```
tp show -- <project>
```

- `project` - the name of the subfolder inside Your templates folder (optional).

```
tp var
```
This will print the `var/ file content.

```
tp var -- svelte
```
This will print the `svelte/var/ file content.


### Config

```
tp config
```

Print the current configuration.
