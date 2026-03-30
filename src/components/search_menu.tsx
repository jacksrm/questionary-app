import {
  CrossCircledIcon,
  MagnifyingGlassIcon,
  PlusIcon,
} from '@radix-ui/react-icons';
import { Tabs } from 'radix-ui';
import { useState } from 'react';
import './search_menu.css';

function formatCPF(value: string): string {
  const digits = value.replace(/\D/g, '').slice(0, 11);

  return digits
    .replace(/^(\d{3})(\d)/, '$1.$2')
    .replace(/^(\d{3})\.(\d{3})(\d)/, '$1.$2.$3')
    .replace(/^(\d{3})\.(\d{3})\.(\d{3})(\d)/, '$1.$2.$3-$4');
}

export default function SearchMenu() {
  const [cpfSearchInput, setCpfSearchInput] = useState('');

  const handleKeyDownCpf = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.ctrlKey || e.metaKey) {
      return;
    }

    // Allow control keys
    if (e.key === 'Tab' || e.key === 'ArrowLeft' || e.key === 'ArrowRight') {
      return;
    }

    e.preventDefault();

    if (e.key === 'Backspace') {
      setCpfSearchInput((prev) => prev.slice(0, -1));
      return;
    }

    if (/\d/.test(e.key)) {
      setCpfSearchInput((prev) => (prev + e.key).slice(0, 11));
    }
  };

  const handlePasteCpf = (e: React.ClipboardEvent<HTMLInputElement>) => {
    e.preventDefault();
    console.log('pasted');

    const pasted = e.clipboardData.getData('text');

    // keep only digits
    const onlyDigits = pasted.replace(/\D/g, '');

    if (!onlyDigits) return;

    setCpfSearchInput(() => onlyDigits.slice(0, 11));
  };

  const resetCpfField = () => setCpfSearchInput('');

  return (
    <div className="flex p-1 gap-1">
      <Tabs.Root defaultValue="name" className="flex flex-col w-full">
        <Tabs.List aria-label="Pesquisa" className="ml-3 flex gap-2">
          <Tabs.Trigger value="name" className="tabs-trigger">
            Nome
          </Tabs.Trigger>
          <Tabs.Trigger value="cpf" className="tabs-trigger">
            CPF
          </Tabs.Trigger>
        </Tabs.List>
        <Tabs.Content value="name">
          <form className="tabs-content-form">
            <input
              type="text"
              placeholder="Ex: João Carlos"
              className="tabs-content-input"
            />
            <button type="button" className="search-menu-button">
              <MagnifyingGlassIcon className="size-6" />
            </button>
          </form>
        </Tabs.Content>
        <Tabs.Content value="cpf">
          <form className="tabs-content-form">
            <div className="flex grow relative">
              <input
                value={formatCPF(cpfSearchInput)}
                onKeyDown={handleKeyDownCpf}
                onPaste={handlePasteCpf}
                type="text"
                placeholder="Ex: 123.456.789-09"
                className="tabs-content-input"
              />
              <button
                onClick={resetCpfField}
                type="button"
                className={`absolute right-2 top-1/2 -translate-y-1/2 transition duration-100 text-red-700 hover:scale-150  active:scale-100 ${cpfSearchInput ? 'scale-100 opacity-100' : 'scale-0 opacity-0 pointer-events-none'}`}>
                <CrossCircledIcon className="" />
              </button>
            </div>
            <button type="button" className="search-menu-button">
              <MagnifyingGlassIcon className="size-6" />
            </button>
          </form>
        </Tabs.Content>
      </Tabs.Root>

      <button type="button" className="search-menu-button mt-auto">
        <PlusIcon className="size-6" />
      </button>
    </div>
  );
}
