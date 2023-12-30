#include <iostream>
#include <vector>

using namespace std;

int main(void) {
	int n;
	cin >> n;

	vector<vector<int>> map;
	for (int i = 0; i < n; i++) {
		map.push_back(vector<int>(i + 1));
		for (int j = 0; j < i + 1; j++) cin >> map[i][j];
	}

	for (int i = n - 2; i >= 0; i--) {
		for (int j = 0; j < i + 1; j++) {
			map[i][j] = max(map[i][j] + map[i + 1][j], map[i][j] + map[i + 1][j + 1]);
		}
	}

	cout << map[0][0];
	return 0;
}
