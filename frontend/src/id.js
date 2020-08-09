let globalID = 0;

export default function nextGlobalID() {
  return globalID++;
}

export function getGlobalID() {
  return globalID;
}