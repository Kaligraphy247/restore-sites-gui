import type { PageLoad } from './$types';
import { getCollection } from '$lib/api/collections';
import { error } from '@sveltejs/kit';

export const load: PageLoad = async ({ params }) => {
    const id = parseInt(params.id);
    
    if (isNaN(id)) {
        throw error(400, 'Invalid collection ID');
    }

    try {
        const collection = await getCollection(id);
        
        if (!collection) {
            throw error(404, 'Collection not found');
        }

        return {
            collection
        };
    } catch (err) {
        console.error('Failed to load collection:', err);
        throw error(500, 'Failed to load collection');
    }
};