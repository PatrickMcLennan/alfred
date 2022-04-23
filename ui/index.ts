import { App } from './App';
import { createRoot } from 'react-dom/client';

import './styles.css';

const ROOT = createRoot(document.querySelector('#ROOT') as HTMLDivElement);

ROOT.render(App());
