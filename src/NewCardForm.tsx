import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { Input } from "./components/ui/input";
import { useQueryClient } from '@tanstack/react-query';

import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select"


const NewCardForm: React.FC = () => {
    const queryClient = useQueryClient();
    const [cardName, setCardName] = useState("");
    const [cardSet, setCardSet] = useState("");

    const saveCard = async () => {
        console.log("saving card", cardName, cardSet);
        invoke("save_card_command", { name: cardName, setCode: cardSet })
            .then((res) => {
                console.debug(res);
                queryClient.invalidateQueries({ queryKey: ['cards'] });
            })
            .catch((err) => {
                console.error(err);
            });
    };

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        saveCard();
    };

    return (
        <form className="space-y-4" onSubmit={handleSubmit}>
            <div className="flex">
                <Input
                    value={cardName}
                    onChange={(e) => setCardName(e.currentTarget.value)}
                    placeholder="Enter card name..."
                    className="w-full p-2 border border-gray-300 rounded mr-2"
                />
                <Select value={cardSet} onValueChange={setCardSet}>
                    <SelectTrigger className="w-[180px] ml-2">
                        <SelectValue placeholder="Set" />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="BLB">BLB</SelectItem>
                        <SelectItem value="BLC">BLC</SelectItem>
                        <SelectItem value="OTJ">OTJ</SelectItem>
                        <SelectItem value="OTC">OTC</SelectItem>
                    </SelectContent>
                </Select>
            </div>
            <Button type="submit" className="w-full p-2 bg-blue-500 text-white rounded">Add Card</Button>
        </form>
    );
};

export default NewCardForm;