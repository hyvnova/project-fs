import { writable } from 'svelte/store';
import { browser } from '$app/environment';


type UserConfig = {
    // Maximun Number of results to show
    result_limit: number
}

// Se inicializa con instancias de History en lugar de arreglos simples
export const default_value: UserConfig = {
    result_limit: 21
};

function load_config(): UserConfig {
    let config = default_value;

    if (browser && localStorage.getItem('user_config')) {
        let plain = JSON.parse(localStorage.getItem('user_config') || '');

        // Comprobar claves faltantes y asignar valores por defecto si es necesario
        for (const key in default_value) {
            if (!(key in plain)) {
                // @ts-ignore
                plain[key] = default_value[key];
            }
        }

        config = plain as unknown as UserConfig;
    }

    return config;
}

const initial_value: UserConfig = load_config();
const user_config = writable(initial_value);

user_config.subscribe((value) => {
    if (browser) {
        // Gracias a toJSON, JSON.stringify convertirá correctamente las instancias de History a arreglos.
        localStorage.setItem('user_config', JSON.stringify(value));
    }
});

export default user_config;
