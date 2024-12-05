import { useQuery, useQueryClient, useMutation } from '@tanstack/react-query';
import { invoke } from '@tauri-apps/api/core';
import { Button } from './components/ui/button';

async function loadCardsCollection() {
    try {
        const res = await invoke('load_cards_command');
        const collection = JSON.parse(res as string);
        console.debug('Loaded cards history:', collection);
        return collection;
    } catch (err) {
        console.error('Error loading cards history:', err);
        return [];
    }
}

async function deleteCard(cardId: string) {
    try {
        const res = await invoke('delete_card_command', { cardId });
        console.debug('Deleted card:', res);
        return res;
    } catch (err) {
        console.error('Error deleting card:', err);
        throw err;
    }
}

function CardsCollection() {
    const queryClient = useQueryClient();
    const listCardsQuery = useQuery({
        queryKey: ['cards'],
        queryFn: loadCardsCollection,
    });

    const deleteCardMutation = useMutation({
        mutationFn: deleteCard,
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ['cards'] });
        },
    });

    const handleDelete = (cardId: string) => {
        deleteCardMutation.mutate(cardId);
    };

    if (listCardsQuery.isLoading) {
        return <div>Loading collection...</div>;
    }

    return (
        <div>
            <ul>
                {listCardsQuery.data?.map((card) => (
                    <li key={card.id} className="flex">
                        <p>{card.name} - {card.set_code}</p>
                        <div className='flex-grow'></div>
                        <Button onClick={() => handleDelete(card.id)}>
                            Delete
                        </Button>
                    </li>
                ))}
            </ul>
        </div>
    );
}

export default CardsCollection;