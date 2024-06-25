class TreeNode:
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def bstToGst(self, root: TreeNode) -> TreeNode:
        self.sum = 0
        self.reverse_inorder(root)
        return root

    def reverse_inorder(self, node: TreeNode):
        if node is not None:
            if node.right is not None:
                self.reverse_inorder(node.right)

            self.sum += node.val
            node.val = self.sum

            if node.left is not None:
                self.reverse_inorder(node.left)
