import { isValidCpf } from './cpf.ts'

export const cpfOk = (d: string) => isValidCpf(d)
