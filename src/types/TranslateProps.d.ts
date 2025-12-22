import { FromLang, ToLang } from './Lang';

export interface TranslateProps {
  from: FromLang;
  to: ToLang;
  text: string;
  translate: string;
}